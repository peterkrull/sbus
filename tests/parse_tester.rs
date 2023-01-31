#[cfg(test)]
mod tests {
    use hex_literal::hex;

    #[test]
    fn test_sbus_parse_original() {

        let mut parser = sbus::SBusPacketParser::new();

        let bytes =
            hex!("00 0F E0 03 1F 58 C0 07 16 B0 80 05 2C 60 01 0B F8 C0 07 00 00 00 00 00 03 00");
        
        parser.push_bytes(&bytes);
        let parsed = parser.try_parse_original().unwrap();

        assert_eq!(
            sbus::SBusPacket {
                channels: [992, 992, 352, 992, 352, 352, 352, 352, 352, 352, 992, 992, 0, 0, 0, 0],
                d1: true,
                d2: true,
                failsafe: false,
                frame_lost: false
            },
            parsed
        );
    }

    #[test]
    fn test_sbus_parse_bitvec() {

        let mut parser = sbus::SBusPacketParser::new();

        let bytes =
            hex!("00 0F E0 03 1F 58 C0 07 16 B0 80 05 2C 60 01 0B F8 C0 07 00 00 00 00 00 03 00");
        
        parser.push_bytes(&bytes);
        let parsed = parser.try_parse().unwrap();

        assert_eq!(
            sbus::SBusPacket {
                channels: [992, 992, 352, 992, 352, 352, 352, 352, 352, 352, 992, 992, 0, 0, 0, 0],
                d1: true,
                d2: true,
                failsafe: false,
                frame_lost: false
            },
            parsed
        );
    }


    #[test]
    fn test_sbus_parse_same() {

        let mut parser = sbus::SBusPacketParser::new();

        let bytes =
            hex!("00 0F E0 03 1F 58 C0 07 16 B0 80 05 2C 60 01 0B F8 C0 07 00 00 00 00 00 03 00");
        
        parser.push_bytes(&bytes);
        let parsed_1 = parser.try_parse().unwrap();

        parser.push_bytes(&bytes);
        let parsed_2 = parser.try_parse_original().unwrap();

        assert_eq!( parsed_1, parsed_2 );
    }
}
