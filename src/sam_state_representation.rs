use std::fmt;

pub trait SamStateRepresentation
where
    Self: fmt::Debug,
{
    type Result;
    type RepresentationData;
    type SamModel;
    type Error;

    fn representation(
        model: Self::SamModel,
        representation_data: Self::RepresentationData,
    ) -> Result<Self::Result, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{tests::SamError, SamStateRepresentation};

    #[derive(Debug, Clone)]
    pub struct LauncherSamModel {
        pub counter: Option<u8>,
        pub started: bool,
        pub launched: bool,
    }

    #[derive(Debug)]
    pub struct LauncherSamState1;

    impl SamStateRepresentation for LauncherSamState1 {
        type Result = ();
        type RepresentationData = Self;
        type SamModel = LauncherSamModel;
        type Error = SamError;

        fn representation(
            data: LauncherSamModel,
            _representation_data: Self,
        ) -> Result<(), SamError> {
            eprintln!("{:?}", data);
            assert_eq!(data.started, false);
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct LauncherSamState2;
    impl SamStateRepresentation for LauncherSamState2 {
        type Result = String;
        type RepresentationData = Self;
        type SamModel = LauncherSamModel;
        type Error = SamError;

        fn representation(
            data: LauncherSamModel,
            _representation_data: Self,
        ) -> Result<String, SamError> {
            Ok(format!("{:?}", data))
        }
    }

    #[test]
    fn representation_unit() {
        let launcher_sam_model = LauncherSamModel {
            counter: Some(10),
            started: false,
            launched: false,
        };
        let _: () = thread::spawn(move || {
            LauncherSamState1::representation(launcher_sam_model, LauncherSamState1)
        })
        .join()
        .unwrap()
        .unwrap();
    }

    #[test]
    fn representation_string() {
        let launcher_sam_model = LauncherSamModel {
            counter: Some(10),
            started: false,
            launched: false,
        };
        let result: String = thread::spawn(move || {
            LauncherSamState2::representation(launcher_sam_model, LauncherSamState2)
        })
        .join()
        .unwrap()
        .unwrap();
        assert_eq!(
            result.as_str(),
            "LauncherSamModel { counter: Some(10), started: false, launched: false }"
        );
    }
}
