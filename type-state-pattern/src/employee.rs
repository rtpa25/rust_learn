pub struct Employee<State> {
    pub name: String,
    pub state: State,
}

pub struct Agreement;
pub struct Signature;
pub struct Training;
pub struct FailedTraining {
    score: u8,
}
pub struct OnboardingComplete {
    score: u8,
}

impl Employee<Agreement> {
    pub fn new(name: String) -> Employee<Agreement> {
        Employee {
            name,
            state: Agreement,
        }
    }

    pub fn read_aggrement(self) -> Employee<Signature> {
        println!("Reading agreement");
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    pub fn sign(self) -> Employee<Training> {
        println!("Signing agreement");
        self.transition(Training)
    }
}

impl Employee<Training> {
    pub fn train(
        self,
        score: u8,
    ) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        println!("Training with score: {}", score);
        if score > 60 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}

impl Employee<FailedTraining> {
    pub fn retry_training(self) -> Employee<Training> {
        println!("Retrying training");
        self.transition(Training)
    }

    pub fn fire(self) {
        println!("Fired!");
    }
}

impl Employee<OnboardingComplete> {
    pub fn welcome(self) {
        println!("Onboarding complete, welcome to the team");
    }
}

impl<State> Employee<State> {
    fn transition<NewState>(self, state: NewState) -> Employee<NewState> {
        Employee {
            name: self.name,
            state,
        }
    }
}
