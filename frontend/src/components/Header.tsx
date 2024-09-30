'use client'

import { useSession } from "next-auth/react";
import Link from "next/link";
import NewCardButton from "./NewCardButton";
import Profile from "./Profile";

export default function Header() {
  const { data: session } = useSession()

  return (
    <div className="navbar bg-neutral sticky top-0">
      <div className="flex-1 text-2xl">
        <Link href="/">
          PokeDB
        </Link>
      </div>
      <NewCardButton></NewCardButton>
      <Profile></Profile>
    </div>
  );
}
