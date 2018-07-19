use super::super::*;

use std::io::Cursor;

proptest! {
    #[test]
    fn read_write(ref input in tcp_any())
    {
        //serialize
        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        input.write(&mut buffer).unwrap();
        //check length
        assert_eq!(input.data_offset as usize * 4, buffer.len());
        //deserialize
        let result = TcpHeader::read(&mut Cursor::new(&buffer)).unwrap();
        //check equivalence
        assert_eq!(input, &result);
    }
}

proptest! {
    #[test]
    fn write_data_offset_too_small(ref base in tcp_any(),
                                   data_offset in 0..TCP_MINIMUM_DATA_OFFSET)
    {
        let mut input = base.clone();
        input.data_offset = data_offset;
        //serialize
        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        assert_matches!(input.write(&mut buffer), Err(
              WriteError::ValueError(_)));
        assert_eq!(0, buffer.len());
    }
}

proptest! {
    #[test]
    fn write_data_offset_too_large(ref base in tcp_any(),
                                   data_offset in (TCP_MAXIMUM_DATA_OFFSET + 1)..255)
    {
        let mut input = base.clone();
        input.data_offset = data_offset;
        //serialize
        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        assert_matches!(input.write(&mut buffer), Err(
              WriteError::ValueError(_)));
    }
}

proptest! {
    #[test]
    fn read_data_offset_too_small(ref input in tcp_any(),
                  data_offset in 0..TCP_MINIMUM_DATA_OFFSET)
    {
        //serialize
        let mut buffer: Vec<u8> = Vec::with_capacity(20);
        input.write(&mut buffer).unwrap();
        //insert the too small data offset into the raw stream
        buffer[12] = (buffer[12] & 0xf) | ((data_offset << 4) & 0xf0);
        //deserialize
        assert_matches!(TcpHeader::read(&mut Cursor::new(&buffer)),
                        Err(ReadError::TcpDataOffsetTooSmall(_)));
    }
}

#[test]
fn eq()
{
    let base = TcpHeader {
        source_port: 1,
        destination_port: 2,
        sequence_number: 3,
        acknowledgment_number: 4,
        data_offset: 5,
        ns: false,
        fin: false,
        syn: false,
        rst: false,
        psh: false,
        ack: false,
        ece: false,
        urg: false,
        cwr: false,
        window_size: 6,
        checksum: 7,
        urgent_pointer: 8,
        options: [0;40]
    };
    //equal
    {
        let other = base.clone();
        assert_eq!(other, base);
    }
    //change every field anc check for neq
    //source_port
    {
        let mut other = base.clone();
        other.source_port = 10;
        assert_ne!(other, base);
    }
    //destination_port
    {
        let mut other = base.clone();
        other.destination_port = 10;
        assert_ne!(other, base);
    }
    //sequence_number
    {
        let mut other = base.clone();
        other.sequence_number = 10;
        assert_ne!(other, base);
    }
    //acknowledgment_number
    {
        let mut other = base.clone();
        other.acknowledgment_number = 10;
        assert_ne!(other, base);
    }
    //data_offset
    {
        let mut other = base.clone();
        other.data_offset = 10;
        assert_ne!(other, base);
    }
    //ns
    {
        let mut other = base.clone();
        other.ns = true;
        assert_ne!(other, base);
    }
    //fin
    {
        let mut other = base.clone();
        other.fin = true;
        assert_ne!(other, base);
    }
    //syn
    {
        let mut other = base.clone();
        other.syn = true;
        assert_ne!(other, base);
    }
    //rst
    {
        let mut other = base.clone();
        other.rst = true;
        assert_ne!(other, base);
    }
    //psh
    {
        let mut other = base.clone();
        other.psh = true;
        assert_ne!(other, base);
    }
    //ack
    {
        let mut other = base.clone();
        other.ack = true;
        assert_ne!(other, base);
    }
    //ece
    {
        let mut other = base.clone();
        other.ece = true;
        assert_ne!(other, base);
    }
    //urg
    {
        let mut other = base.clone();
        other.urg = true;
        assert_ne!(other, base);
    }
    //cwr
    {
        let mut other = base.clone();
        other.cwr = true;
        assert_ne!(other, base);
    }
    //window_size
    {
        let mut other = base.clone();
        other.window_size = 10;
        assert_ne!(other, base);
    }
    //checksum
    {
        let mut other = base.clone();
        other.checksum = 10;
        assert_ne!(other, base);
    }
    //urgent_pointer
    {
        let mut other = base.clone();
        other.urgent_pointer = 10;
        assert_ne!(other, base);
    }
    //options (first element)
    {
        let mut other = base.clone();
        other.options[0] = 10;
        assert_ne!(other, base);
    }
    //options (last element)
    {
        let mut other = base.clone();
        other.options[39] = 10;
        assert_ne!(other, base);
    }
}

proptest! {
    #[test]
    fn debug_fmt(ref input in tcp_any())
    {
        assert_eq!(&format!("TcpHeader {{ source_port: {}, destination_port: {}, sequence_number: {}, acknowledgment_number: {}, data_offset: {}, ns: {}, fin: {}, syn: {}, rst: {}, psh: {}, ack: {}, ece: {}, urg: {}, cwr: {}, window_size: {}, checksum: {}, urgent_pointer: {} }}",
                input.source_port,
                input.destination_port,
                input.sequence_number,
                input.acknowledgment_number,
                input.data_offset,
                input.ns,
                input.fin,
                input.syn,
                input.rst,
                input.psh,
                input.ack,
                input.ece,
                input.urg,
                input.cwr,
                input.window_size,
                input.checksum,
                input.urgent_pointer
            ),
            &format!("{:?}", input)
        );
    }
}