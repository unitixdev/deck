import AuthProvider from "@/components/AuthProvider";
import Header from "@/components/Header";
import type { Metadata } from "next";

import "./globals.scss"
import Footer from "@/components/Footer";

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
        <AuthProvider>
          <Header />
          {children}
          <Footer />
        </AuthProvider>
      </body>
    </html>
  );
}
