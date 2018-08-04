#![feature(test)]

extern crate nordint;

#[cfg(test)]
mod biguint_tests {
    use nordint::BigUint;
    use std::str::FromStr;

    #[test]
    fn biguint_empty() {
        let number = BigUint::empty();
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
        assert_eq!(10, number.capacity());
    }

    #[test]
    fn biguint_with_capacity() {
        let number = BigUint::with_capacity(50);
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
        assert_eq!(50, number.capacity());
    }

    #[test]
    fn biguint_zero() {
        let number = BigUint::zero();
        assert_eq!("BigUint { buckets: [0] }", format!("{:?}", number));
    }

    #[test]
    fn biguint_one() {
        let number = BigUint::one();
        assert_eq!("BigUint { buckets: [1] }", format!("{:?}", number));
    }

    #[test]
    fn biguint_default() {
        let number = BigUint::default();
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));
    }

    #[test]
    fn biguint_new() {
        let number = BigUint::new("");
        assert_eq!("BigUint { buckets: [] }", format!("{:?}", number));

        let number = BigUint::new("123456789000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123_456_789_000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123,456,789,000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("000123456789000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("00000000000000000000000000123456789000");
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123456789123456789_123456789123456789");
        assert_eq!(
            "BigUint { buckets: [123456789123456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("123456789123456789_023456789023456789_000000000000000001");
        assert_eq!(
            "BigUint { buckets: [1, 23456789023456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("100000000000000000_000000000000000000_000000000000000001");
        assert_eq!(
            "BigUint { buckets: [1, 0, 100000000000000000] }",
            format!("{:?}", number)
        );

        let number = BigUint::new("1_000000000000000000_000000000000000000_000000000000000011");
        assert_eq!(
            "BigUint { buckets: [11, 0, 0, 1] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn biguint_from_str() {
        let number = BigUint::from_str("").unwrap_or_default();
        assert_eq!(format!("{:?}", BigUint::default()), format!("{:?}", number));

        let number = BigUint::from_str("123,456,789,000").unwrap_or_default();
        assert_eq!(format!("{:?}", BigUint::default()), format!("{:?}", number));

        let number = BigUint::from_str("123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("000123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number =
            BigUint::from_str("00000000000000000000000000123456789000").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789000] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("123456789123456789123456789123456789").unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [123456789123456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("123456789123456789023456789023456789000000000000000001")
            .unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [1, 23456789023456789, 123456789123456789] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("100000000000000000000000000000000000000000000000000001")
            .unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [1, 0, 100000000000000000] }",
            format!("{:?}", number)
        );

        let number = BigUint::from_str("1000000000000000000000000000000000000000000000000000011")
            .unwrap_or_default();
        assert_eq!(
            "BigUint { buckets: [11, 0, 0, 1] }",
            format!("{:?}", number)
        );
    }

    #[test]
    fn biguint_to_string() {
        let number = BigUint::empty();
        assert_eq!("", number.to_string());

        let number = BigUint::zero();
        assert_eq!("0", number.to_string());

        let number = BigUint::one();
        assert_eq!("1", number.to_string());

        let number = BigUint::new("123,456,789,000");
        assert_eq!("123456789000", number.to_string());

        let number = BigUint::new("123456789123456789_123456789123456789");
        assert_eq!("123456789123456789123456789123456789", number.to_string());

        let number = BigUint::new("123456789123456789_023456789023456789_000000000000000001");
        assert_eq!(
            "123456789123456789023456789023456789000000000000000001",
            number.to_string()
        );

        let number = BigUint::new("100000000000000000_000000000000000000_000000000000000001");
        assert_eq!(
            "100000000000000000000000000000000000000000000000000001",
            number.to_string()
        );

        let number = BigUint::new("1_000000000000000000_000000000000000000_000000000000000011");
        assert_eq!(
            "1000000000000000000000000000000000000000000000000000011",
            number.to_string()
        );

        let number = BigUint::new("182378728712487128471297918269128659182591287591285619256976595761928576192785619587612985614951629857634895761345610495710297851092485709128560192854609128589127540981265409812746509182750981273409812367409812375098312650938127409182750938125609283740918274012937840912378501923865091287401974019235601927850129385701285601298374012984710238471092586091238741209874091286509123874019287409128560912785019285601285655");
        assert_eq!(number.to_string(), "182378728712487128471297918269128659182591287591285619256976595761928576192785619587612985614951629857634895761345610495710297851092485709128560192854609128589127540981265409812746509182750981273409812367409812375098312650938127409182750938125609283740918274012937840912378501923865091287401974019235601927850129385701285601298374012984710238471092586091238741209874091286509123874019287409128560912785019285601285655");
    }

    #[test]
    fn biguint_add_assign() {
        let mut lhs = BigUint::one();
        let rhs = BigUint::one();
        lhs += &rhs;
        assert_eq!(BigUint::new("2"), lhs);

        let mut lhs = BigUint::new("999_999_999_999_999_999");
        let rhs = BigUint::one();
        lhs += &rhs;
        assert_eq!(BigUint::new("1_000_000_000_000_000_000"), lhs);

        let mut lhs = BigUint::new("999_999_999_999_999_999");
        let rhs = BigUint::new("999_999_999_999_999_999");
        let expected = BigUint::new("1_999_999_999_999_999_998");
        lhs += &rhs;
        assert_eq!(expected, lhs);

        let mut lhs = BigUint::new(
            "[999_999_999_999_999_999] [999_999_999_999_999_999] [999_999_999_999_999_999]",
        );
        let rhs = BigUint::new(
            "[999_999_999_999_999_999] [999_999_999_999_999_999] [999_999_999_999_999_999]",
        );
        let expected = BigUint::new(
            "1_999_999_999_999_999_999_999_999_999_999_999_999_999_999_999_999_999_998",
        );
        lhs += &rhs;
        assert_eq!(expected, lhs);

        let mut lhs = BigUint::new(
            "[123_456_789_987_654_321] [123_456_789_987_654_321] [123_456_789_987_654_321]",
        );
        let rhs = BigUint::new(
            "[987_654_321_123_456_789] [987_654_321_123_456_789] [987_654_321_123_456_789]",
        );
        let expected = BigUint::new(
            "1_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_110",
        );
        lhs += &rhs;
        assert_eq!(expected, lhs);

        let mut lhs = BigUint::new(
            "[123_456_789_987_654_321] [123_456_789_987_654_321] [123_456_789_987_654_321]",
        );
        let rhs = BigUint::one();
        let expected =
            BigUint::new("123_456_789_987_654_321_123_456_789_987_654_321_123_456_789_987_654_322");
        lhs += &rhs;
        assert_eq!(expected, lhs);

        let mut lhs = BigUint::one();
        let rhs = BigUint::new(
            "[123_456_789_987_654_321] [123_456_789_987_654_321] [123_456_789_987_654_321]",
        );
        let expected =
            BigUint::new("123_456_789_987_654_321_123_456_789_987_654_321_123_456_789_987_654_322");
        lhs += &rhs;
        assert_eq!(expected, lhs);
    }

    #[test]
    fn biguint_fib() {
        let expected = BigUint::empty();
        assert_eq!(BigUint::fib(0), expected);

        let first = BigUint::new("52");
        let second = BigUint::one();
        let expected = BigUint::new("52");
        assert_eq!(expected, BigUint::fib_generic(first, second, 1));

        let first = BigUint::one();
        let second = BigUint::new("102");
        let expected = BigUint::new("102");
        assert_eq!(expected, BigUint::fib_generic(first, second, 2));

        let first = BigUint::new("5");
        let second = BigUint::new("6");
        let expected = BigUint::new("28");
        assert_eq!(expected, BigUint::fib_generic(first, second, 5));

        let expected = BigUint::new("832040");
        assert_eq!(expected, BigUint::fib(30));

        let expected =
            BigUint::new("222232244629420445529739893461909967206666939096499764990979600");
        assert_eq!(expected, BigUint::fib(300));

        let expected = BigUint::new("20793608237133498072112648988642836825087036094015903119682945866528501423455686648927456034305226515591757343297190158010624794267250973176133810179902738038231789748346235556483191431591924532394420028067810320408724414693462849062668387083308048250920654493340878733226377580847446324873797603734794648258113858631550404081017260381202919943892370942852601647398213554479081823593715429566945149312993664846779090437799284773675379284270660175134664833266377698642012106891355791141872776934080803504956794094648292880566056364718187662668970758537383352677420835574155945658542003634765324541006121012446785689171494803262408602693091211601973938229446636049901531963286159699077880427720289235539329671877182915643419079186525118678856821600897520171070499437657067342400871083908811800976259727431820539554256869460815355918458253398234382360435762759823179896116748424269545924633204614137992850814352018738480923581553988990897151469406131695614497783720743461373756218685106856826090696339815490921253714537241866911604250597353747823733268178182198509240226955826416016690084749816072843582488613184829905383150180047844353751554201573833105521980998123833253261228689824051777846588461079790807828367132384798451794011076569057522158680378961532160858387223882974380483931929541222100800313580688585002598879566463221427820448492565073106595808837401648996423563386109782045634122467872921845606409174360635618216883812562321664442822952537577492715365321134204530686742435454505103269768144370118494906390254934942358904031509877369722437053383165360388595116980245927935225901537634925654872380877183008301074569444002426436414756905094535072804764684492105680024739914490555904391369218696387092918189246157103450387050229300603241611410707453960080170928277951834763216705242485820801423866526633816082921442883095463259080471819329201710147828025221385656340207489796317663278872207607791034431700112753558813478888727503825389066823098683355695718137867882982111710796422706778536913192342733364556727928018953989153106047379741280794091639429908796650294603536651238230626");
        assert_eq!(expected, BigUint::fib(9999));
    }

}