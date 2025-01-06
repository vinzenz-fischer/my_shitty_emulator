use std::collections::{HashMap, LinkedList};

use crate::{Arg, Instruction};

pub struct Assembler;

impl Assembler {
    pub fn assemble(assembly: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        let mut context = AssemblerContext::new();
        let mut problems: Vec<AssemblerError> = Vec::new();
        let mut bytecode: Vec<u8> = Vec::new();
        
        for (i, chr) in assembly.to_lowercase().chars().enumerate() {
            if chr != '\r' {
                println!("{} {:?}", context.state, chr);
            }

            match chr {
                '\r' => continue, // fucky windows thing    >:( grrrr
                '\n' => {
                    context.line_start = i+1;
                    context.line += 1;
                    println!("    context.line_start: {}, context.line: {}, ", context.line_start, context.line);
                }
                _ => {}
            }

            context.state = match context.state {
                AssemblerState::Default => match chr {
                    ' ' | '\t' | '\n' => context.state,
                    '/' => AssemblerState::AnyCommentStart,
                    '@' => AssemblerState::MarkerDefinitionStart,
                    'a'..='z' => AssemblerState::Instruction,
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: format!("Expected marker definition or instruction, but got {chr:?}.")
                        });
                        context.state
                    }
                }
                AssemblerState::AnyCommentStart => match chr {
                    '/' => AssemblerState::SingleLineComment,
                    '*' => AssemblerState::MultiLineComment,
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: "Did you want to write a comment `// …` or multi-line comment `/* … */`?".to_owned()
                        }); 
                        context.state
                    }
                }
                AssemblerState::SingleLineComment => match chr {
                    '\n' => AssemblerState::Default,
                    _ => context.state
                }
                AssemblerState::MultiLineComment => match chr {
                    '*' => AssemblerState::MultiLineCommentEnd,
                    _ => context.state
                }
                AssemblerState::MultiLineCommentEnd => match chr {
                    '/' => AssemblerState::Default,
                    _ => AssemblerState::MultiLineComment
                }
                AssemblerState::MarkerDefinitionStart => match chr {
                    'a'..='z' | '_' => AssemblerState::MarkerDefinition(false),
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: format!("Sorry, can't have '{chr}' as the first character of a marker.")
                        });
                        context.state
                    }
                }
                AssemblerState::MarkerDefinition(whitespace_only) => match chr {
                    'a'..='z' | '_' | '0'..='9' if whitespace_only => context.state,
                    ' ' |'\t' => AssemblerState::MarkerDefinition(true),
                    '\n' => {
                        let curr_word = assembly[context.word_start..i].to_owned();
                        match context.markers.contains_key(&curr_word) {
                            true => todo!("Cannot define a marker more than once (First definition at ...)"),
                            false => context.markers.insert(curr_word, i),
                        };
                        AssemblerState::Default
                    },
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: "Allowed characters in markers are 'a'-'z', '_' and '0'-'9'.".to_owned()
                        }); 
                        context.state
                    }
                }
                AssemblerState::Instruction => match chr {
                    'a'..='z' => AssemblerState::Instruction,
                    ' ' | '\t' | '\n' => match Instruction::from_slice(&assembly[context.word_start..i]) {
                        Some(instruction) => {
                            bytecode.push(instruction.to_opcode().unwrap());
                            context.args_queue.extend(Instruction::get_arguments(instruction));
                            match context.args_queue.pop_front() {
                                Some(Arg::Any) => AssemblerState::AnyArgument,
                                Some(Arg::Register) => AssemblerState::RegisterStart,
                                None => AssemblerState::Default,
                            }
                        }
                        None => {
                            problems.push(AssemblerError::UnexpectedCharacter{
                                context: context.clone(),
                                info: format!("Invalid instruction \"{}\"", &assembly[context.word_start..i]).to_owned()
                            });
                            context.state
                        }
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: "All instruction consist of the letters A-Z (case-insensitive)".to_owned()
                        });
                        context.state
                    }
                }
                AssemblerState::AnyArgument => match chr { // could be Register, char literal, number literal, marker reference
                    ' ' | '\t' => AssemblerState::AnyArgument, // ignore whitespace
                    '\'' => AssemblerState::CharacterLiteral,
                    '0'..='9' => AssemblerState::AnyNumberLiteral(
                        chr.to_digit(10).unwrap() as u8
                        // Don't need to handle overflow here because this is just the first digit.
                    ),
                    '@' => AssemblerState::MarkerReference,
                    'r' => AssemblerState::RegisterStart,
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: "Expected any type of argument, got newline instead.".to_owned()
                        });
                        AssemblerState::Default
                    }
                }
                AssemblerState::MarkerReference => match chr {
                    ' ' | '\t' | '\n' => {
                        let curr_word = assembly[context.word_start..i].to_owned();
                        match context.markers.contains_key(&curr_word) {
                            true => {
                                let marker_address = context.markers.get(&curr_word).unwrap();
                                bytecode.push(*marker_address as u8);
                            },
                            false => {
                                problems.push(AssemblerError::UndefinedMarker {
                                    context: context.clone(),
                                    info: format!("Undefined marker \"{}\"", curr_word)
                                });
                            }
                        }

                        match context.args_queue.pop_front() {
                            Some(Arg::Any) => AssemblerState::AnyArgument,
                            Some(Arg::Register) => AssemblerState::RegisterStart,
                            None => AssemblerState::Default,
                        }
                    }
                    _ => AssemblerState::MarkerReference
                }
                AssemblerState::CharacterLiteral => AssemblerState::CharacterLiteralEnd(chr),
                AssemblerState::CharacterLiteralEnd(c) => match chr {
                    '\'' => {
                        bytecode.push(c as u8);
                        AssemblerState::AnyArgument // is this how you skip to the next arg?
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: format!("Expected ', got {chr} instead.")
                        });
                        AssemblerState::Default
                    }
                }
                AssemblerState::AnyNumberLiteral(a) => match chr {
                    '0'..='9' => AssemblerState::AnyNumberLiteral({ // Decimal by default
                        const BASE: u32 = 10;
                        a * BASE as u8 + chr.to_digit(BASE).unwrap() as u8
                        // [FIXME] handle overflow
                    }),
                    'b' if a == 0 => AssemblerState::BinNumberLiteral(0),
                    'd' if a == 0 => AssemblerState::DecNumberLiteral(0),
                    'o' if a == 0 => AssemblerState::OctNumberLiteral(0),
                    'x' if a == 0 => AssemblerState::HexNumberLiteral(0),
                    ' ' | '\t' => AssemblerState::AnyArgument, // is this how you skip to the next arg?
                    '\n' => {
                        bytecode.push(a);
                        AssemblerState::Default
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: "Expected any type of number literal, got newline instead.".to_owned()
                        });
                        AssemblerState::Default
                    }
                }
                AssemblerState::BinNumberLiteral(a) => match chr {
                    '0'..='1' => AssemblerState::BinNumberLiteral({
                        const BASE: u32 = 2;
                        a * BASE as u8 + chr.to_digit(BASE).unwrap() as u8
                        // [FIXME] handle overflow
                    }),
                    ' ' | '\t' | '\n' => {
                        bytecode.push(a);
                        match context.args_queue.pop_front() {
                            Some(Arg::Any) => AssemblerState::AnyArgument,
                            Some(Arg::Register) => AssemblerState::RegisterStart,
                            None => AssemblerState::Default,
                        }
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: format!("Expected binary number literal, got {chr:?} instead.")
                        });
                        AssemblerState::Default
                    }
                }
                AssemblerState::OctNumberLiteral(a) => match chr {
                    '0'..='8' => AssemblerState::OctNumberLiteral({
                        const BASE: u32 = 8;
                        a * BASE as u8 + chr.to_digit(BASE).unwrap() as u8
                        // [FIXME] handle overflow
                    }),
                    ' ' | '\t' | '\n' => {
                        bytecode.push(a);
                        match context.args_queue.pop_front() {
                            Some(Arg::Any) => AssemblerState::AnyArgument,
                            Some(Arg::Register) => AssemblerState::RegisterStart,
                            None => AssemblerState::Default,
                        }
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: format!("Expected octal number literal, got {chr:?} instead.")
                        });
                        AssemblerState::Default
                    }
                }
                AssemblerState::DecNumberLiteral(a) => match chr {
                    '0'..='9' => AssemblerState::DecNumberLiteral({
                        const BASE: u32 = 10;
                        a * BASE as u8 + chr.to_digit(BASE).unwrap() as u8
                        // [FIXME] handle overflow
                    }),
                    ' ' | '\t' | '\n' => {
                        bytecode.push(a);
                        match context.args_queue.pop_front() {
                            Some(Arg::Any) => AssemblerState::AnyArgument,
                            Some(Arg::Register) => AssemblerState::RegisterStart,
                            None => AssemblerState::Default,
                        }
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: format!("Expected decimal number literal, got {chr:?} instead.")
                        });
                        AssemblerState::Default
                    }
                }
                AssemblerState::HexNumberLiteral(a) => match chr {
                    '0'..='9' => AssemblerState::HexNumberLiteral({
                        const BASE: u32 = 16;
                        a * BASE as u8 + chr.to_digit(BASE).unwrap() as u8
                        // [FIXME] handle overflow
                    }),
                    'a'..='f' => AssemblerState::HexNumberLiteral({
                        const BASE: u32 = 16;
                        a * BASE as u8 + chr.to_digit(BASE).unwrap() as u8
                        // [FIXME] handle overflow
                    }),
                    ' ' | '\t' | '\n' => {
                        bytecode.push(a);
                        match context.args_queue.pop_front() {
                            Some(Arg::Any) => AssemblerState::AnyArgument,
                            Some(Arg::Register) => AssemblerState::RegisterStart,
                            None => AssemblerState::Default,
                        }
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info: format!("Expected hexadecimal number literal, got {chr:?} instead.")
                        });
                        AssemblerState::Default
                    }
                }
                _ => todo!("Unhandled state! {}", context.state)
            };
            
            match chr {
                ' ' | '\t' => context.word_start = i + 1,
                _ => {}
            }
        }
    
        match problems.len() == 0 {
            true => Ok(bytecode),
            false => Err(problems)
        }
    }
}

#[derive(Debug)]
pub struct AssemblerContext {
    line: usize,
    line_start: usize,
    word_start: usize,
    word_end: usize,
    state: AssemblerState,
    args_queue: LinkedList<Arg>,
    markers: HashMap<String, usize>,
}

impl AssemblerContext {
    pub fn new() -> Self {
        Self {
            line: 0,
            line_start: 0,
            word_start: 0,
            word_end: 0,
            state: AssemblerState::Default,
            args_queue: LinkedList::new(),
            markers: HashMap::new(),
        }
    }
}

impl Clone for AssemblerContext {
    fn clone(&self) -> Self {
        Self {
            line:       self.line.clone(),
            line_start: self.line_start.clone(),
            word_start: self.word_start.clone(),
            word_end:   self.word_end.clone(),
            state:      self.state.clone(),
            // LinkedList<T> does not implement the Clone trait :(
            // Is there a way to impl traits for foreign types?
            // Or should this just not be included in the AssemblerContext?
            args_queue: LinkedList::new(),
            markers:    self.markers.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AssemblerState {
    Default,
    AnyCommentStart,
    SingleLineComment,
    MultiLineComment,
    MultiLineCommentEnd,
    MarkerDefinitionStart,
    MarkerDefinition(bool), // The bool indicates that the marker already has a name, so now we expect only whitespace to follow.
    Instruction,
    AnyArgument,
    RegisterStart,
    CharacterLiteral,
    CharacterLiteralEnd(char),
    MarkerReference,
    AnyNumberLiteral(u8),
    BinNumberLiteral(u8), // Prefix: 0b
    OctNumberLiteral(u8), // Prefix: 0o
    DecNumberLiteral(u8), // Prefix: 0d
    HexNumberLiteral(u8), // Prefix: 0x
}

impl std::fmt::Display for AssemblerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<34}",
            match self {
                Self::Default                 => "Default",
                Self::AnyCommentStart         => "AnyCommentStart",
                Self::SingleLineComment       => "SingleLineComment",
                Self::MultiLineComment        => "MultiLineComment",
                Self::MultiLineCommentEnd     => "MultiLineCommentEnd",
                Self::MarkerDefinitionStart   => "MarkerDefinitionStart",
                Self::MarkerDefinition(false) => "MarkerDefinition",
                Self::MarkerDefinition(true)  => "MarkerDefinition (whitespace only)",
                Self::Instruction             => "Instruction",
                Self::AnyArgument             => "AnyArgument",
                Self::RegisterStart           => "RegisterStart",
                Self::CharacterLiteral        => "CharacterLiteral",
                Self::CharacterLiteralEnd(_)  => "CharacterLiteralEnd (_)", // [TODO] Also provide the character.
                Self::MarkerReference         => "MarkerReference",
                Self::AnyNumberLiteral(_)     => "AnyNumberLiteral (_)", // [TODO] Also provide the number.
                Self::BinNumberLiteral(_)     => "BinNumberLiteral (_)", // [TODO] Also provide the number.
                Self::OctNumberLiteral(_)     => "OctNumberLiteral (_)", // [TODO] Also provide the number.
                Self::DecNumberLiteral(_)     => "DecNumberLiteral (_)", // [TODO] Also provide the number.
                Self::HexNumberLiteral(_)     => "HexNumberLiteral (_)", // [TODO] Also provide the number.
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum AssemblerError {
    UnexpectedCharacter       { context: AssemblerContext, info: String },
    UnknownToken              { context: AssemblerContext, info: String },
    MarkerDefinedMultipleTimes{ context: AssemblerContext, info: String },
    UndefinedMarker           { context: AssemblerContext, info: String },
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // [TODO] Make these error messages look nice
        match self {
            AssemblerError::UnexpectedCharacter{context, info} => {
                write!(f,
                    "Error: Unexpected character on line {} (from {} to {}; State: {})\n| {}",
                    context.line, context.word_start, context.word_end, context.state, info
                )
            }
            AssemblerError::UnknownToken{context, info} => {
                write!(f,
                    "Error: Unknown token on line {} (from {} to {}; State: {})\n| {}",
                    context.line, context.word_start, context.word_end, context.state, info
                )
            }
            AssemblerError::UndefinedMarker{context, info} => {
                write!(f,
                    "Error: Undefined marker on line {} (from {} to {}; State: {})\n| {}",
                    context.line, context.word_start, context.word_end, context.state, info
                )
            }
            AssemblerError::MarkerDefinedMultipleTimes{context, info} => {
                write!(f,
                    "Error: Marker defined more than once. (Line {})\n| {}",
                    context.line, info
                )
            }
        }
    }
}
