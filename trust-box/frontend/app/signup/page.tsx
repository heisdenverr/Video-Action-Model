'use client';

import { useState } from 'react';
import { authSchema } from '@/lib/validation';

export default function SignupPage() {
  const [message, setMessage] = useState('');
  async function onSubmit(formData: FormData) {
    const payload = {
      email: formData.get('email'),
      password: formData.get('password'),
    };
    const parsed = authSchema.safeParse(payload);
    if (!parsed.success) return setMessage('Invalid input');
    setMessage('Account created (hook backend /auth/signup).');
  }

  return (
    <main className="mx-auto max-w-md p-4">
      <h1 className="mb-4 text-2xl font-bold">Sign up</h1>
      <form action={onSubmit} className="space-y-3">
        <input name="email" type="email" placeholder="Email" className="w-full rounded border border-slate-700 bg-slate-900 p-2" />
        <input name="password" type="password" placeholder="Password" className="w-full rounded border border-slate-700 bg-slate-900 p-2" />
        <button className="w-full rounded bg-emerald-500 p-2 font-semibold text-black">Create account</button>
      </form>
      <p className="mt-2 text-sm text-slate-300">{message}</p>
    </main>
  );
}
