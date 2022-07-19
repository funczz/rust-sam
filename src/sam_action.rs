use std::fmt;

pub trait SamAction
where
    Self: fmt::Debug,
{
    type SamModel;
    type ActionData;
    type Error;

    fn execute(
        present: fn(Self::ActionData) -> Result<Self::SamModel, Self::Error>,
        data: Self::ActionData,
    ) -> Result<Self::SamModel, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{
        tests::{self, SamError},
        SamAction,
    };

    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum LauncherState {
        COUNTING,
        LAUNCHED,
    }

    #[derive(Debug, Clone)]
    pub struct LauncherModel {
        pub state: LauncherState,
        pub count: Option<u8>,
    }

    pub struct LauncherData {
        model: LauncherModel,
    }

    pub struct LauncherPresent;

    impl LauncherPresent {
        pub fn present(data: LauncherData) -> Result<LauncherModel, SamError> {
            Ok(data.model)
        }
    }

    #[derive(Debug, Clone)]
    pub struct ConvertAction;

    impl SamAction for ConvertAction {
        type SamModel = LauncherModel;
        type ActionData = LauncherData;
        type Error = SamError;
        fn execute(
            present: fn(LauncherData) -> Result<LauncherModel, tests::SamError>,
            data: LauncherData,
        ) -> Result<LauncherModel, SamError> {
            present(data)
        }
    }

    #[test]
    fn execute() {
        let data = LauncherData {
            model: LauncherModel {
                state: LauncherState::COUNTING,
                count: Some(10),
            },
        };
        let result = thread::spawn(move || ConvertAction::execute(LauncherPresent::present, data))
            .join()
            .unwrap();
        match result {
            Ok(v) => assert_eq!(v.count.unwrap(), 10),
            Err(e) => panic!("{}", e),
        }
    }
}
