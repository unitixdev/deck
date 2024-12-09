import NextAuth from "next-auth";
import ZITADEL from "next-auth/providers/zitadel";

export const {
  handlers: { GET, POST },
  signIn,
  signOut,
  auth,
} = NextAuth({
  providers: [
    ZITADEL({
      issuer: process.env.ZITADEL_ISSUER!,
      authorization: {
        params: {
          scope: `openid email profile urn:zitadel:iam:org:project:id${process.env.ZITADEL_PROJECT_ID}:aud`,
        },
      },
    }),
  ],
  callbacks: {
    async session({ session, token }) {
      session.user.id = token.sub!;
      return session;
    },

    async jwt({ token, account, profile }) {
      if (account) {
        token.accessToken = account.access_token;
        token.provider = account.provider;
      }

      if (profile) {
        token.zitadelProfile = profile;
      }

      return token;
    },
  },
});
