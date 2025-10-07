module TestAccount::PatternWildcards {
    struct Point { x: u64, y: u64, z: u64 } has drop;

    struct Config {
        threshold: u64,
        multiplier: u64,
        enabled: bool,
        max_value: u64,
    } has drop, copy;

    fun get_x_coordinate(p: Point): u64 {
        let Point { x, .. } = p;
        x
    }

    fun compute_from_config(c: Config, value: u64): u64 {
        let Config { threshold, multiplier, .. } = c;
        if (value > threshold) {
            value * multiplier
        } else {
            value
        }
    }

    fun is_x_greater_than_y(p: Point): bool {
        let Point { x, y, .. } = p;
        x > y
    }

    fun sum_threshold_and_multiplier(c: Config): u64 {
        let Config { threshold, multiplier, .. } = c;
        threshold + multiplier
    }

    #[test]
    fun test_get_x_coordinate() {
        let p1 = Point { x: 10, y: 20, z: 30 };
        assert!(get_x_coordinate(p1) == 10);

        let p2 = Point { x: 100, y: 0, z: 0 };
        assert!(get_x_coordinate(p2) == 100);
    }

    #[test]
    fun test_compute_from_config() {
        let config = Config {
            threshold: 50,
            multiplier: 2,
            enabled: true,
            max_value: 1000,
        };

        assert!(compute_from_config(config, 30) == 30);
        assert!(compute_from_config(config, 60) == 120);
        assert!(compute_from_config(config, 100) == 200);
    }

    #[test]
    fun test_is_x_greater_than_y() {
        assert!(is_x_greater_than_y(Point { x: 10, y: 5, z: 0 }) == true);
        assert!(is_x_greater_than_y(Point { x: 5, y: 10, z: 0 }) == false);
        assert!(is_x_greater_than_y(Point { x: 5, y: 5, z: 0 }) == false);
    }

    #[test]
    fun test_sum_threshold_and_multiplier() {
        let config = Config {
            threshold: 10,
            multiplier: 20,
            enabled: false,
            max_value: 500,
        };
        assert!(sum_threshold_and_multiplier(config) == 30);
    }
}
