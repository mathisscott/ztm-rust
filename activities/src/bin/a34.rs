// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoad       (luggage is loaded onto correct plane)
//   * OffLoad      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug)]
enum State {
    CheckIn,
    OnLoad,
    OffLoad,
    AwaitingPickup,
    EndCustody,
}

#[derive(Debug)]
struct Luggage<State> {
    tracking_id: u32,
    state: State,
}
impl Luggage<State> {
    fn new(id: u32) -> Luggage<CheckIn> {
        Luggage {
            tracking_id: id,
            state: CheckIn,
        }
    }

    fn transition<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage {
            tracking_id: self.tracking_id,
            state: state,
        }
    }
}

impl Luggage<CheckIn> {
    fn load_in(self) -> Luggage<OnLoad> {
        Luggage {
            tracking_id: self.tracking_id,
            state: OnLoad,
        }
    }
}

impl Luggage<OnLoad> {
    fn load_out(self) -> Luggage<OffLoad> {
        Luggage {
            tracking_id: self.tracking_id,
            state: OffLoad,
        }
    }
}

impl Luggage<OffLoad> {
    fn deliver(self) -> Luggage<AwaitingPickup> {
        Luggage {
            tracking_id: self.tracking_id,
            state: AwaitingPickup,
        }
    }
}

impl Luggage<AwaitingPickup> {
    fn picked_up(self) -> Luggage<EndCustody> {
        Luggage {
            tracking_id: self.tracking_id,
            state: EndCustody,
        }
    }
}

#[derive(Debug)]
struct CheckIn;

#[derive(Debug)]
struct OnLoad;

#[derive(Debug)]
struct OffLoad;

#[derive(Debug)]
struct AwaitingPickup;

#[derive(Debug)]
struct EndCustody;


fn main() {
    let suitcase = Luggage::new(49);
    let end = suitcase.load_in().load_out().deliver().picked_up();
    println!("{end:?}");
}
