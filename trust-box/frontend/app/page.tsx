import Link from 'next/link';

export default function Home() {
  return (
    <main className="mx-auto max-w-md space-y-4 p-4">
      <h1 className="text-3xl font-bold">Trust-Box</h1>
      <p className="text-slate-300">Secure micro-escrow built for Nigerian freelancers and clients.</p>
      <div className="grid grid-cols-2 gap-3">
        <Link className="rounded bg-emerald-500 px-3 py-2 text-center font-medium text-black" href="/signup">Sign up</Link>
        <Link className="rounded border border-slate-700 px-3 py-2 text-center" href="/login">Login</Link>
      </div>
      <Link className="block rounded border border-slate-700 px-3 py-2 text-center" href="/dashboard">Open Dashboard</Link>
    </main>
  );
}
