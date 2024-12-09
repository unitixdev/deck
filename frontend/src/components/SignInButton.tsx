"use server"
import { signIn } from "@/auth"

export default async function SignInButton() {
    async function login() {
        return await signIn('zitadel')
    }

    return (
        <button onClick={login}>
            Sign in with ZITADEL
          </button>
    )
}