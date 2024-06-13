pub struct Luggage<State> {
    pub id: u32,
    pub weight: u32,
    pub state: State,
}

pub struct CheckIn;
pub struct OnLoading;
pub struct OffLoading;
pub struct AwaitingPickup;
pub struct EndCustody;

impl Luggage<CheckIn> {
    pub fn check_in(id: u32, weight: u32) -> Luggage<CheckIn> {
        Luggage {
            id,
            weight,
            state: CheckIn,
        }
    }

    pub fn load(self) -> Luggage<OnLoading> {
        println!("Loading luggage");
        self.transition(OnLoading)
    }
}

impl Luggage<OnLoading> {
    pub fn offload(self) -> Luggage<OffLoading> {
        println!("Offloading luggage");
        self.transition(OffLoading)
    }
}

impl Luggage<OffLoading> {
    pub fn await_pickup(self) -> Luggage<AwaitingPickup> {
        println!("Awaiting pickup");
        self.transition(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    pub fn end_custody(self) -> Luggage<EndCustody> {
        println!("Ending custody");
        self.transition(EndCustody)
    }
}

impl<State> Luggage<State> {
    fn transition<NewState>(self, state: NewState) -> Luggage<NewState> {
        Luggage {
            id: self.id,
            weight: self.weight,
            state,
        }
    }
}
