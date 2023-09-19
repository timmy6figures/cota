import styles from "./Board.module.css"

export type CellProps = {
    cell: string | null; // "Horse", null, "Kite" etc
}
export const Cell: React.FC<CellProps> = ({ cell }) => {
    return (
        <div className={styles.cell}>
            <div className={styles["cell-fragment"]}></div>
            <div className={styles["cell-fragment"]}></div>
            <div className={styles["cell-fragment"]}></div>
            <div className={styles["cell-fragment"]}></div>
        </div>
    );
}
