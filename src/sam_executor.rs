use std::fmt;

pub trait SamExecutor
where
    Self: fmt::Debug,
{
    type Result;
    type RepresentationData;
    type SamModel;
    type ActionData;
    type Error;

    fn sam_present() -> fn(Self::ActionData) -> Result<Self::SamModel, Self::Error>;

    fn sam_state_next_action() -> fn(Self::SamModel) -> Result<Self::SamModel, Self::Error>;

    fn sam_state_representation(
    ) -> fn(Self::SamModel, Self::RepresentationData) -> Result<Self::Result, Self::Error>;

    fn do_action(
        action: fn(
            fn(Self::ActionData) -> Result<Self::SamModel, Self::Error>,
            Self::ActionData,
        ) -> Result<Self::SamModel, Self::Error>,
        data: Self::ActionData,
    ) -> Result<Self::SamModel, Self::Error> {
        let result = action(Self::sam_present(), data);
        let model = match result {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let result = Self::sam_state_next_action()(model);
        let model = match result {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(model)
    }

    fn do_representation(
        model: Self::SamModel,
        representation_data: Self::RepresentationData,
    ) -> Result<Self::Result, Self::Error> {
        Self::sam_state_representation()(model, representation_data)
    }

    fn execute(
        action: fn(
            fn(Self::ActionData) -> Result<Self::SamModel, Self::Error>,
            Self::ActionData,
        ) -> Result<Self::SamModel, Self::Error>,
        data: Self::ActionData,
        representation_data: Self::RepresentationData,
    ) -> Result<Self::Result, Self::Error> {
        let result = Self::do_action(action, data);
        let model = match result {
            Ok(v) => v,
            Err(v) => return Err(v),
        };
        Self::do_representation(model, representation_data)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        tests::SamError, SamAction, SamModelPresent, SamStateNextAction, SamStateRepresentation,
    };

    use super::SamExecutor;

    //
    // sam action input  data
    //
    #[derive(Debug, Clone)]
    struct FooActionData {
        value: String,
    }
    impl FooActionData {
        fn new(value: String) -> FooActionData {
            FooActionData { value }
        }
    }

    //
    // sam model
    //
    #[derive(Debug, Clone)]
    struct FooSamModel {
        value: String,
    }
    impl FooSamModel {
        fn new(value: String) -> FooSamModel {
            FooSamModel { value }
        }
    }
    //
    // sam model present
    //
    impl SamModelPresent for FooSamModel {
        type ActionData = FooActionData;
        type Error = SamError;

        fn present(data: FooActionData) -> Result<Self, SamError> {
            Ok(FooSamModel::new(data.value))
        }
    }

    //
    // sam action
    //
    #[derive(Debug)]
    struct FooSamAction;
    impl SamAction for FooSamAction {
        type SamModel = FooSamModel;
        type ActionData = FooActionData;
        type Error = SamError;

        fn execute(
            present: fn(FooActionData) -> Result<FooSamModel, SamError>,
            data: FooActionData,
        ) -> Result<FooSamModel, SamError> {
            present(data)
        }
    }

    //
    //
    // sam state
    #[derive(Debug)]
    struct FooSamState;
    //
    // sam state next action
    //
    impl SamStateNextAction for FooSamState {
        type SamModel = FooSamModel;
        type Error = SamError;

        fn next_action_predicate(
            data: crate::NextActionData<FooSamModel>,
        ) -> Result<crate::NextActionData<FooSamModel>, SamError> {
            Ok(crate::NextActionData::new_terminate(FooSamModel::new(
                data.get().value.to_uppercase(),
            )))
        }
    }
    //
    // sam state representation
    //
    #[derive(Debug)]
    struct FooSamStateRepresentation;
    impl SamStateRepresentation for FooSamStateRepresentation {
        type Result = ();
        type RepresentationData = Self;
        type SamModel = FooSamModel;
        type Error = SamError;

        fn representation(
            model: FooSamModel,
            _representation: FooSamStateRepresentation,
        ) -> Result<(), SamError> {
            assert_eq!(model.value.as_str(), "HELLO WORLD.");
            Ok(())
        }
    }

    //
    // sam executor
    //
    #[derive(Debug)]
    struct FooSamExcutor;
    impl SamExecutor for FooSamExcutor {
        type Result = ();
        type RepresentationData = FooSamStateRepresentation;
        type SamModel = FooSamModel;
        type ActionData = FooActionData;
        type Error = SamError;

        fn sam_present() -> fn(FooActionData) -> Result<FooSamModel, SamError> {
            FooSamModel::present
        }

        fn sam_state_next_action() -> fn(FooSamModel) -> Result<FooSamModel, SamError> {
            FooSamState::next_action
        }

        fn sam_state_representation(
        ) -> fn(FooSamModel, FooSamStateRepresentation) -> Result<(), SamError> {
            FooSamStateRepresentation::representation
        }
    }

    #[test]
    fn do_action() {
        let data = FooActionData::new(String::from("hello world."));
        let result = FooSamExcutor::do_action(FooSamAction::execute, data);
        match result {
            Ok(v) => assert_eq!(v.value.as_str(), "HELLO WORLD."),
            Err(v) => panic!("{}", v),
        }
    }

    #[test]
    fn do_representation() {
        let model = FooSamModel::new(String::from("HELLO WORLD."));
        let result = FooSamExcutor::do_representation(model, FooSamStateRepresentation);
        match result {
            Ok(v) => assert_eq!(v, ()),
            Err(v) => panic!("{}", v),
        }
    }

    #[test]
    fn execute() {
        let data = FooActionData::new(String::from("hello world."));
        let result = FooSamExcutor::execute(FooSamAction::execute, data, FooSamStateRepresentation);
        match result {
            Ok(v) => assert_eq!(v, ()),
            Err(v) => panic!("{}", v),
        }
    }
}
