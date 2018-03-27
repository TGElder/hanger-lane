import City;

struct MapCell {
    x: usize,
    y: usize,
    content: str,
}

impl MapCell {
    fn parse(&self) -> MapSetupInstruction {
    }
}

struct MapSetup {
    roads: Vec<Road>;
    sources: Vec<usize>,
    destinations: Vec<usize>,
}

trait MapSetupInstruction {
    fn setup(map_setup: MapSetup) -> MapSetup;
}

struct CityMap {

}


