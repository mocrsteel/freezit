import {Inter, Geist} from 'next/font/google'
import React from "react";
import type {Metadata} from 'next'

import '@/globals.css'

import NavBar from "@components/nav-bar";

const font = Geist({subsets: ['latin']})

export const metadata: Metadata = {
  title: 'Freezit',
  description: 'A freezer storage management app',
}

const RootLayout = ({
  children,
  modal,
}: {
  children: React.ReactNode
  modal: React.ReactNode
}) => {
  return (
    <html lang="en">
    <body className={font.className}>
    <div id={"dialogs-root"}/>
    {modal}
    <NavBar/>
    <main id="content" className="p-4 m-0">
      {children}
    </main>
    </body>
    </html>
  )
}

export default RootLayout
