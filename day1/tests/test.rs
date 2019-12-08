extern crate day1;

#[cfg(test)]
mod test_module {

    use day1::functions;
    
    #[test]
    fn test1() {
        assert_eq!(2.0, functions::module_counting::count_fuel_needed(&12));
    }
    
    #[test]
    fn test2() {
        assert_eq!(2.0, functions::module_counting::count_fuel_needed(&14));
    }
    
    #[test]
    fn test3() {
        assert_eq!(654.0, functions::module_counting::count_fuel_needed(&1969));
    }
    
    #[test]
    fn test4() {
        assert_eq!(33583.0, functions::module_counting::count_fuel_needed(&100756));
    }
}