import './globals.css';
import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Trust-Box',
  description: 'Micro-escrow for freelancers and clients',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="min-h-screen">{children}</body>
    </html>
  );
}
