use super::*;
use super::position;

#[cfg(test)]
pub mod tests {
    
    
    #[test]
    fn rank_and_file_of_position() {
        use crate::cota::position;
        let pos = position::A1;
        assert_eq!(pos.get_file(), 0);
        assert_eq!(pos.get_rank(), 0);
        let pos = position::C4;
        assert_eq!(pos.get_file(), 2);
        assert_eq!(pos.get_rank(), 3);
        let pos = position::I10;
        assert_eq!(pos.get_file(), 8);
        assert_eq!(pos.get_rank(), 9);
    }

    #[test]
    fn rank_and_file_as_string() {
        use crate::cota::position;
        let pos = position::A1;
        assert_eq!(pos.to_string(), String::from("A1"));
        let pos = position::C4;
        assert_eq!(pos.to_string(), String::from("C4"));
        let pos = position::I10;
        assert_eq!(pos.to_string(), String::from("IT"));
    }

    #[test]
    fn position_to_direction() {
        use crate::cota::position;
        let pos = position::A1;
        assert_eq!(pos.up().unwrap(), position::A2);
        let pos = position::A1;
        assert_eq!(pos.right().unwrap(), position::B1);
        let pos = position::A1;
        assert_eq!(pos.upper_right().unwrap(), position::B2);
    }

    #[test]
    fn position_out_of_bounds() {
        use crate::cota::position;
        let pos1 = position::A1;
        assert_ne!(pos1.up(), None);
        assert_eq!(pos1.down(), None);
        assert_eq!(pos1.left(), None);
        assert_ne!(pos1.right(), None);
        let pos2 = position::A10;
        assert_eq!(pos2.up(), None);
        assert_ne!(pos2.down(), None);
        assert_eq!(pos2.left(), None);
        assert_ne!(pos2.right(), None);
        let pos3 = position::I1;
        assert_ne!(pos3.up(), None);
        assert_eq!(pos3.down(), None);
        assert_ne!(pos3.left(), None);
        assert_eq!(pos3.right(), None);
        let pos4 = position::I10;
        assert_eq!(pos4.up(), None);
        assert_ne!(pos4.down(), None);
        assert_ne!(pos4.left(), None);
        assert_eq!(pos4.right(), None);
    }
}

