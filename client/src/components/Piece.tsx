import styles from "./Board.module.css"

export type PieceProps = {
    pieceName: string | null; // if null, the cell is a blank.
}

export const Piece: React.FC<PieceProps> = ({pieceName}) => {
    return (
        <img src={`pieces/${pieceName === null ? 'cannon' : pieceName}.png`} className={styles.piece}></img>
    )
}
