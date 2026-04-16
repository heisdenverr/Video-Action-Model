import Link from 'next/link';
import { Card } from '@/components/card';

export default function Dashboard() {
  return (
    <main className="mx-auto max-w-2xl space-y-4 p-4">
      <h1 className="text-2xl font-bold">Freelancer Dashboard</h1>
      <Card title="Create Trust Link">
        <p className="text-sm text-slate-300">Create a fixed-price escrow link and share with a client.</p>
      </Card>
      <Card title="Active Transactions">
        <ul className="space-y-2 text-sm">
          <li className="rounded border border-slate-800 p-2">TBX-2026-001 · FUNDED · ₦50,000</li>
          <li className="rounded border border-slate-800 p-2">TBX-2026-002 · IN_PROGRESS · ₦75,000</li>
        </ul>
      </Card>
      <Link href="/pay/demo-transaction" className="block rounded bg-indigo-500 px-3 py-2 text-center font-semibold">Open Payment Page</Link>
    </main>
  );
}
