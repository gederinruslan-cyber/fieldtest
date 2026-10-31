import type { Metadata } from "next";
import { EquityCalculator } from "./EquityCalculator";
import styles from "../calculator.module.css";

export const metadata: Metadata = {
  title: "Equity calculator · Fieldtest",
  description:
    "All-in equity for two to nine poker hands or ranges, preflop or on any board: exact enumeration where it fits, otherwise Monte Carlo with its margin shown. A free study tool.",
};

export default function EquityPage() {
  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <p className={styles.kicker}>Free study tool</p>
        <h1>Equity calculator</h1>
        <p className={styles.lede}>
          Give each player a hand or a range, add a board if there is one, and see how often each
          wins the pot at showdown. Up to nine players.
        </p>
      </header>
      <EquityCalculator />
      <section className={styles.notes} aria-labelledby="how">
        <h2 id="how">How the numbers work</h2>
        <p>
          A deal gives every player one combo from their range, no two sharing a card, weighted by
          the combos' weights; every way to complete the board is equally likely. When every deal
          and board can be counted in under half a second, they all are and the result is exact: a
          hand against a hand or a small range, and most ranges on the turn and river or on a flop
          when they are not both wide. Otherwise the calculator samples 200,000 deals from the same
          distribution and labels the result an estimate, with one standard error beside each
          equity.
        </p>
        <p>
          Equity is the share of the pot a player expects: wins, plus their part of every split. Win
          counts only pots taken alone; tie counts every split pot.
        </p>
      </section>
    </main>
  );
}
