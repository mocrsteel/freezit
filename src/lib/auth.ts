import NextAuth from 'next-auth'
import {Auth} from "@auth/core";

export const { handlers, signIn, signOut } = NextAuth ({
  providers: ['google']
})
