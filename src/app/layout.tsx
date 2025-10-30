import {Inter, Geist} from 'next/font/google'
import React from "react";
import type {Metadata} from 'next'

import '@/app/globals.css'

import NavBar from "@/components/nav-bar";

const font = Geist({subsets: ['latin']})

export const metadata: Metadata = {
  title: 'Freezit',
  description: 'A freezer storage management app',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
    <body className={font.className}>
    <NavBar/>
    <main id="content" className="p-6 md:p-4 m-0">
      {children}
    </main>
    </body>
    </html>
  )
}
