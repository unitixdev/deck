'use client'

import { getSession, signOut, signIn } from "next-auth/react";
import { useEffect, useState } from "react";

export default function Home() {
  const [isAuth, setAuth] = useState<boolean>()
  const [user, setUser] = useState<{
    name?: string | null
    email?: string | null
    image?: string | null
  } | null>(null)

  const login = async () => {
    await signIn('zitadel')
  }

  const logout = async () => {
    await signOut({ redirect: true, redirectTo: 'https://localhost:3000' })
  }

  useEffect(() => {
    async function fetchSession() {
      const session = await getSession()
      setAuth(!!session?.user)

      if (!!session?.user) {
        setUser(session?.user ?? null)
        console.log(session)
      }
    }

    fetchSession()
  }, [])

  return (
    <>
    { !isAuth && <button onClick={login}>Login with Zitadel</button>
    }
    { isAuth && <>
      <p>Hello {user?.name}</p>
      <button onClick={logout}>Logout with Zitadel</button>
    </>
    }
    </>
  );
}
