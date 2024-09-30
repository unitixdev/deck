import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "PokeDB",
  description: "Your trusty digital TCG collection app!",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`antialiased`}
      >
        {children}
      </body>
    </html>
  );
}
