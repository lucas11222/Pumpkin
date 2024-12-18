use async_trait::async_trait;
use pumpkin_protocol::client::play::{
    CommandSuggestion, ProtoCmdArgParser, ProtoCmdArgSuggestionType,
};
use pumpkin_world::block::block_registry::{self, Block};

use crate::{command::dispatcher::CommandError, server::Server};

use super::{
    super::{
        args::{ArgumentConsumer, RawArgs},
        CommandSender,
    },
    Arg, DefaultNameArgConsumer, FindArg, GetClientSideArgParser,
};

pub(crate) struct BlockArgumentConsumer;

impl GetClientSideArgParser for BlockArgumentConsumer {
    fn get_client_side_parser(&self) -> ProtoCmdArgParser {
        ProtoCmdArgParser::Resource {
            identifier: "block",
        }
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<ProtoCmdArgSuggestionType> {
        None
    }
}

#[async_trait]
impl ArgumentConsumer for BlockArgumentConsumer {
    async fn consume<'a>(
        &self,
        _sender: &CommandSender<'a>,
        _server: &'a Server,
        args: &mut RawArgs<'a>,
    ) -> Option<Arg<'a>> {
        let s = args.pop()?;
        Some(Arg::Block(s))
    }

    async fn suggest<'a>(
        &self,
        _sender: &CommandSender<'a>,
        _server: &'a Server,
        _input: &'a str,
    ) -> Result<Option<Vec<CommandSuggestion<'a>>>, CommandError> {
        Ok(None)
    }
}

impl DefaultNameArgConsumer for BlockArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "block"
    }

    fn get_argument_consumer(&self) -> &dyn ArgumentConsumer {
        self
    }
}

impl<'a> FindArg<'a> for BlockArgumentConsumer {
    type Data = &'a Block;

    fn find_arg(args: &'a super::ConsumedArgs, name: &'a str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::Block(name)) => match block_registry::get_block(name) {
                Some(block) => Ok(block),
                None => Err(CommandError::GeneralCommandIssue(format!(
                    "Block {name} does not exist."
                ))),
            },
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}
