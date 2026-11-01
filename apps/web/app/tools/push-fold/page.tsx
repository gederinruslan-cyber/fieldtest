import type { Metadata } from "next";
import styles from "../calculator.module.css";
import { PushFoldCalculator } from "./PushFoldCalculator";

export const metadata: Metadata = {
  title: "Push/fold Nash calculator · Fieldtest",
  description:
    "Heads-up push/fold Nash ranges for the small blind and big blind, with ICM over every stack at the table. Exact equities, free and unlimited.",
};

export default function PushFoldPage() {
  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <p className={styles.kicker}>Free study tool</p>
        <h1>Push/fold Nash calculator</h1>
        <p className={styles.lede}>
          Everyone has folded to the small blind, who either moves all in or folds. See which hands
          the small blind should push and which the big blind should call, at equilibrium.
        </p>
      </header>
      <PushFoldCalculator />
      <section className={styles.notes} aria-labelledby="how">
        <h2 id="how">How the numbers work</h2>
        <p>
          Every all-in between two hands is counted exactly: each pair of the 169 starting hands,
          over every board, with the cards the other hand holds removed. Each result (a fold, a
          steal, a win, a tie or a loss) is valued by ICM over every stack at the table, so other
          players who have folded still count.
        </p>
        <p>
          The ranges are found by fictitious play: each player keeps answering the other's average
          strategy until neither can gain more than a tiny, stated share of the prize pool by
          changing any hand. A hand shaded with stripes is played part of the time.
        </p>
        <p>
          Push/fold is a model for short stacks: no one raises less than all in, and the big blind's
          only choice is to call or fold. With more players acting, use it as a guide.
        </p>
      </section>
    </main>
  );
}
