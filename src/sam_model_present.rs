use std::fmt;

pub trait SamModelPresent
where
    Self: fmt::Debug + Sized,
{
    type ActionData;
    type Error;

    fn present(data: Self::ActionData) -> Result<Self, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{tests::SamError, SamModelPresent};

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

    #[derive(Debug, Clone)]
    pub struct LauncherSamModel {
        pub counter: Option<u8>,
        pub started: bool,
        pub launched: bool,
    }

    pub struct LauncherData {
        model: LauncherModel,
    }

    impl SamModelPresent for LauncherSamModel {
        type ActionData = LauncherData;
        type Error = SamError;

        fn present(data: LauncherData) -> Result<LauncherSamModel, SamError> {
            let (started, launched) = match data.model.state {
                LauncherState::COUNTING => (true, false),
                LauncherState::LAUNCHED => (true, true),
            };
            let counter = data.model.count;
            Ok(LauncherSamModel {
                counter,
                started,
                launched,
            })
        }
    }

    #[test]
    fn present_state_counting() {
        let data = LauncherData {
            model: LauncherModel {
                state: LauncherState::COUNTING,
                count: Some(10),
            },
        };
        let result = thread::spawn(move || LauncherSamModel::present(data))
            .join()
            .unwrap();
        match result {
            Ok(v) => assert_eq!((v.counter, v.started, v.launched), (Some(10), true, false)),
            _ => panic!("ERROR."),
        }
    }

    #[test]
    fn present_state_launched() {
        let data = LauncherData {
            model: LauncherModel {
                state: LauncherState::LAUNCHED,
                count: Some(10),
            },
        };
        let result = thread::spawn(move || LauncherSamModel::present(data))
            .join()
            .unwrap();
        match result {
            Ok(v) => assert_eq!((v.counter, v.started, v.launched), (Some(10), true, true)),
            _ => panic!("ERROR."),
        }
    }
}
