pub mod brain {
    pub mod v1 {
        tonic::include_proto!("echo.vtuber.brain.v1");
    }
}

pub mod voice {
    pub mod v1 {
        tonic::include_proto!("echo.vtuber.voice.v1");
    }
}
