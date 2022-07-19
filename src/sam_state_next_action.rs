use std::fmt;

use crate::NextActionData;

pub trait SamStateNextAction
where
    Self: fmt::Debug,
{
    type SamModel;
    type Error;

    fn next_action_predicate(
        data: NextActionData<Self::SamModel>,
    ) -> Result<NextActionData<Self::SamModel>, Self::Error>;

    fn next_action(model: Self::SamModel) -> Result<Self::SamModel, Self::Error> {
        let mut data = NextActionData::new_continue(model);
        while let NextActionData::Continue { model: _ } = &data {
            let result = Self::next_action_predicate(data);
            data = match result {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
        }
        Ok(data.get())
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{tests::SamError, NextActionData, SamStateNextAction};

    #[derive(Debug, Clone)]
    pub struct LauncherSamModel {
        pub counter: Option<u8>,
        pub started: bool,
        pub launched: bool,
    }

    #[derive(Debug)]
    pub struct LauncherSamState;

    impl SamStateNextAction for LauncherSamState {
        type SamModel = LauncherSamModel;
        type Error = SamError;

        fn next_action_predicate(
            data: NextActionData<LauncherSamModel>,
        ) -> Result<NextActionData<LauncherSamModel>, SamError> {
            //data が NextActionData::Terminate の場合はそのまま値を返す
            let model = match &data {
                NextActionData::Continue { model } => model,
                NextActionData::Terminate { model: _ } => return Ok(data),
            };
            // 本来は SamState の状態に応じて SamAction::execute を実行する。
            // ここでは、テスト向けに処理を全て内包している。
            let mut new = model.clone();
            let result = match (model.counter, model.started, model.launched) {
                //launch
                (Some(v), true, false) if v == 0 => {
                    new.launched = true;
                    NextActionData::new_terminate(new)
                }
                // decrement
                (Some(v), true, false) => {
                    new.counter = Some(v - 1);
                    NextActionData::new_continue(new)
                }
                // start
                (Some(_), false, false) => {
                    new.started = true;
                    NextActionData::new_terminate(new)
                }
                (_, _, _) => NextActionData::new_terminate(new),
            };
            Ok(result)
        }
    }

    #[test]
    fn next_action_start() {
        let launcher_sam_model = LauncherSamModel {
            counter: Some(10),
            started: false,
            launched: false,
        };
        let result = thread::spawn(move || LauncherSamState::next_action(launcher_sam_model))
            .join()
            .unwrap();
        match result {
            Ok(v) => assert_eq!(v.started, true),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn next_action_counting() {
        let launcher_sam_model = LauncherSamModel {
            counter: Some(10),
            started: true,
            launched: false,
        };
        let result = thread::spawn(move || LauncherSamState::next_action(launcher_sam_model))
            .join()
            .unwrap();
        match result {
            Ok(v) => assert_eq!(v.counter.unwrap(), 0),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn next_action_launch() {
        let launcher_sam_model = LauncherSamModel {
            counter: Some(0),
            started: true,
            launched: false,
        };
        let result = thread::spawn(move || LauncherSamState::next_action(launcher_sam_model))
            .join()
            .unwrap();
        match result {
            Ok(v) => assert_eq!(v.launched, true),
            Err(e) => panic!("{}", e),
        }
    }
}
