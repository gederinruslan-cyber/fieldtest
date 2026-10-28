import Link from "next/link";

export default function Home() {
  return (
    <main>
      <h1>Fieldtest</h1>
      <p>
        Practice tournament poker against a field you configure, then study ICM-graded feedback.
        Fieldtest is a study tool for use away from the table.
      </p>
      <p>
        <Link href="/tools/range-analyzer">Range analyzer</Link>: break a range down on any board.
      </p>
    </main>
  );
}
