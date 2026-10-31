import type { Metadata } from "next";
import styles from "../calculator.module.css";
import { IcmCalculator } from "./IcmCalculator";

export const metadata: Metadata = {
  title: "ICM calculator · Fieldtest",
  description:
    "Exact Malmuth-Harville ICM for up to 20 players: what each tournament stack is worth in prize money, next to its share of the chips. A free study tool.",
};

export default function IcmPage() {
  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <p className={styles.kicker}>Free study tool</p>
        <h1>ICM calculator</h1>
        <p className={styles.lede}>
          Enter each player's chips and the payouts, and see what every stack is worth in prize
          money. Up to 20 players.
        </p>
      </header>
      <IcmCalculator />
      <section className={styles.notes} aria-labelledby="how">
        <h2 id="how">How the numbers work</h2>
        <p>
          The Independent Chip Model (Malmuth-Harville) gives each player the chance of finishing
          first equal to their share of the chips. Once someone has taken a place, the rest share
          the next place the same way, by their share of the chips still in play. A player's equity
          is the payouts weighted by the chance of each finish, so the equities add up to the prize
          pool. Every finishing order is counted, so the result is exact.
        </p>
        <p>
          ICM is a model: it ignores blinds, position and skill. Compare a stack's prize share with
          its chip share to see how much less a chip is worth as the stack grows.
        </p>
      </section>
    </main>
  );
}
