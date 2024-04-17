import { Inter } from 'next/font/google'
import React from "react";
import type { Metadata } from 'next'

import '@fortawesome/fontawesome-svg-core/styles.css'
import '@styles/globals.scss'

import NavBar from "@components/nav-bar";
import Modal from "@components/modal";

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Freezit',
  description: 'A freezer storage management app',
}

const RootLayout = ({
  children,
}: {
  children: React.ReactNode
}) => {
  return (
    <html lang="en">
      <body className={inter.className + ' body-container'}>
        <div id={"modal-root"} />
        <NavBar />
        {children}
      </body>
    </html>
  )
}

export default RootLayout
