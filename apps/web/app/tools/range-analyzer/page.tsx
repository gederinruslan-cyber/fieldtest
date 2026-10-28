import type { Metadata } from "next";
import styles from "./analyzer.module.css";
import { RangeAnalyzer } from "./RangeAnalyzer";

export const metadata: Metadata = {
  title: "Range analyzer · Fieldtest",
  description:
    "Break a poker range down on any flop, turn or river: made hands and draws with exact combo counts after card removal. A free study tool.",
};

export default function RangeAnalyzerPage() {
  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <p className={styles.kicker}>Free study tool</p>
        <h1>Range analyzer</h1>
        <p className={styles.lede}>
          Build a range on the grid or type it, pick a board, and see what the range holds: made
          hands and draws, counted combo by combo after card removal. Keep the hands you would
          continue with and carry them to the next street.
        </p>
      </header>
      <RangeAnalyzer />
      <section className={styles.notes} aria-labelledby="how">
        <h2 id="how">How the counts work</h2>
        <p>
          Every combo in the range that shares no card with the board is enumerated, so counts and
          shares are exact, with no sampling. A weighted combo counts at its weight. Each combo
          takes one made-hand class, the first that applies from straight flush down to no made
          hand. Draws are counted on the flop and turn only, and only when a hole card helps: four
          to a flush, an open-ended straight draw (two or more ranks complete it) or a gutshot (one
          rank).
        </p>
      </section>
    </main>
  );
}
