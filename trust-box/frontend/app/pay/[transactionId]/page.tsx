import { notFound } from 'next/navigation';

export default function PaymentPage({ params }: { params: { transactionId: string } }) {
  if (!params.transactionId) notFound();

  return (
    <main className="mx-auto max-w-md space-y-4 p-4">
      <h1 className="text-2xl font-bold">Pay Trust Link</h1>
      <p className="text-slate-300">Transaction: {params.transactionId}</p>
      <div className="rounded-xl border border-slate-800 p-4">
        <p>Amount: ₦50,000</p>
        <p>Rail: Card / Bank Transfer / USSD</p>
        <button className="mt-3 w-full rounded bg-emerald-500 p-2 font-semibold text-black">Pay with Paystack</button>
      </div>
    </main>
  );
}
