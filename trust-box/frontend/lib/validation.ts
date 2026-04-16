import { z } from 'zod';

export const authSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
});

export const createLinkSchema = z.object({
  amountNgn: z.coerce.number().positive(),
  deliveryDescription: z.string().min(10).max(500),
  feePercent: z.coerce.number().min(2.5).max(5),
});
