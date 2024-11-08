use crate::Instruction;

pub struct Assembler;

impl Assembler {
    pub fn assemble(assembly: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        let mut context = AssemblerContext::new();
        let mut problems: Vec<AssemblerError> = Vec::new();
        // let mut tokens: Vec<T: Token>;
        
        for (i, chr) in assembly.to_lowercase().chars().enumerate() {
            println!("{} {:?}", context.state, chr);
            match chr {
                '\r' => continue, // fucky windows thing    >:( grrrr
                '\n' => {
                    context.line_start = i+1;
                    context.line += 1;
                },
                ' ' | '\t' => context.word_start = i+1,
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
                            info: format!("Expected marker definition or instruction, but got {chr}.")
                        });
                        context.state
                    }
                },
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
                },
                AssemblerState::SingleLineComment => match chr {
                    '\n' => AssemblerState::Default,
                    _ => context.state
                },
                AssemblerState::MultiLineComment => match chr {
                    '*' => AssemblerState::MultiLineCommentEnd,
                    _ => context.state
                },
                AssemblerState::MultiLineCommentEnd => match chr {
                    '/' => AssemblerState::Default,
                    _ => AssemblerState::MultiLineComment
                },
                AssemblerState::MarkerDefinitionStart => match chr {
                    'a'..='z' | '_' => AssemblerState::MarkerDefinition(false),
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info:format!("Sorry, can't have '{chr}' as the first character of a marker.")
                        });
                        context.state
                    }
                },
                AssemblerState::MarkerDefinition(whitespace_only) => match chr {
                    'a'..='z' |'_' | '0'..='9' if whitespace_only => context.state,
                    ' ' |'\t' => AssemblerState::MarkerDefinition(true),
                    '\n' => {
                        // TODO: add marker definition to `markers`, error if it already exists
                        AssemblerState::Default
                    },
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info:"Allowed characters in markers are 'a'-'z', '_' and '0'-'9'.".to_owned()
                        }); 
                        context.state
                    }
                },
                AssemblerState::Instruction => match chr {
                    'a'..='z' => AssemblerState::Instruction,
                    ' ' | '\t' | '\n' => match Instruction::from_slice(&assembly[context.word_start..i]) {
                        /* TODO:
                        instruction from_slice(last word)
                        get instruction args
                        queue args
                        AssemblerState::ArgumentQueue
                        */
                        
                        _ => todo!("queue arguments")
                    }
                    _ => {
                        problems.push(AssemblerError::UnexpectedCharacter{
                            context: context.clone(),
                            info:"All instruction consist of letters 'a'-'z' (case-insensitive)".to_owned()
                        }); 
                        context.state
                    }
                },
                _ => todo!("Unhandled state! {}", context.state)
            };
        }

        let mut bytecode = Vec::new();

        todo!("add stuff to `bytecode`");
    
        match problems.len() == 0 {
            true => Ok(bytecode),
            false => Err(problems)
        }
    }
}

pub struct AssemblerContext {
    line: usize,
    line_start: usize,
    word_start: usize,
    word_end: usize,
    state: AssemblerState,
}

impl AssemblerContext {
    pub fn new() -> Self {
        Self {
            line: 0,
            line_start: 0,
            word_start: 0,
            word_end: 0,
            state: AssemblerState::Default,
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
        }
    }
}

pub enum AssemblerState {
    Default,
    AnyCommentStart,
    SingleLineComment,
    MultiLineComment,
    MultiLineCommentEnd,
    MarkerDefinitionStart,
    MarkerDefinition(bool),
    Instruction,
    ArgumentQueue,
    AnyArgument,
    RegisterStart,
    CharacterLiteral,
    CharacterLiteralEnd,
    MarkerReferenceStart,
    MarkerReference,
    AnyNumberLiteral,
    BinNumberLiteral,
    OctNumberLiteral,
    DecNumberLiteral,
    HexNumberLiteral,
}

impl Clone for AssemblerState {
    fn clone(&self) -> Self {
        match self {
            Self::Default               => Self::Default,
            Self::AnyCommentStart       => Self::AnyCommentStart,
            Self::SingleLineComment     => Self::SingleLineComment,
            Self::MultiLineComment      => Self::MultiLineComment,
            Self::MultiLineCommentEnd   => Self::MultiLineCommentEnd,
            Self::MarkerDefinitionStart => Self::MarkerDefinitionStart,
            Self::MarkerDefinition(b) => Self::MarkerDefinition(b.clone()),
            Self::Instruction           => Self::Instruction,
            Self::ArgumentQueue         => Self::ArgumentQueue,
            Self::AnyArgument           => Self::AnyArgument,
            Self::RegisterStart         => Self::RegisterStart,
            Self::CharacterLiteral      => Self::CharacterLiteral,
            Self::CharacterLiteralEnd   => Self::CharacterLiteralEnd,
            Self::MarkerReferenceStart  => Self::MarkerReferenceStart,
            Self::MarkerReference       => Self::MarkerReference,
            Self::AnyNumberLiteral      => Self::AnyNumberLiteral,
            Self::BinNumberLiteral      => Self::BinNumberLiteral,
            Self::OctNumberLiteral      => Self::OctNumberLiteral,
            Self::DecNumberLiteral      => Self::DecNumberLiteral,
            Self::HexNumberLiteral      => Self::HexNumberLiteral,
            
        }
    }
}

impl std::fmt::Display for AssemblerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:^16}",
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
                Self::ArgumentQueue           => "ArgumentQueue",
                Self::AnyArgument             => "AnyArgument",
                Self::RegisterStart           => "RegisterStart",
                Self::CharacterLiteral        => "CharacterLiteral",
                Self::CharacterLiteralEnd     => "CharacterLiteralEnd",
                Self::MarkerReferenceStart    => "MarkerReferenceStart",
                Self::MarkerReference         => "MarkerReference",
                Self::AnyNumberLiteral        => "AnyNumberLiteral",
                Self::BinNumberLiteral        => "BinNumberLiteral",
                Self::OctNumberLiteral        => "OctNumberLiteral",
                Self::DecNumberLiteral        => "DecNumberLiteral",
                Self::HexNumberLiteral        => "HexNumberLiteral",
            }
        )
    }
}

pub enum AssemblerError {
    UnexpectedCharacter       {context: AssemblerContext, info: String},
    UnknownToken              {context: AssemblerContext, info: String},
    MarkerDefinedMultipleTimes{context: AssemblerContext, info: String},
    MarkerDefinedStupidName   {context: AssemblerContext, info: String},
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Make these error messages look nice
        match self {
            AssemblerError::UnexpectedCharacter{context, info} => {
                write!(f, "Error: Unexpected character on line {} (from {} to {}; State: {})\n| {}",
                    context.line, context.word_start, context.word_end, context.state, info
                )
            },
            AssemblerError::UnknownToken{context, info} => {
                write!(f, "Error: Unknown token on line {} (from {} to {}; State: {})\n| {}",
                    context.line, context.word_start, context.word_end, context.state, info
                )
            },
            AssemblerError::MarkerDefinedMultipleTimes{context, info} => {
                write!(f, "Error: Marker defined more than once. (Line {})\n| {}", context.line, info)
            },
            AssemblerError::MarkerDefinedStupidName{context, info} => {
                write!(f, "Error: That marker you're defining on line {} doesn't have a very descriptive name, does it?\n| {}",
                    context.line, info
                )
            },
        }
    }
}

pub trait Token: Sized {
    fn to_bytecode(&self) -> Vec<u8>;
}
