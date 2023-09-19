import styles from "./Board.module.css"

export const River: React.FC = () => {
    return (
        <div className={`${styles.river} ${styles.cell}`}></div>
    )
}