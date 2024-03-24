import '@styles/globals.scss'
import '@fortawesome/fontawesome-svg-core/styles.css'
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import React from "react";
import NavBar from "@ui/nav-bar";

const inter = Inter({ subsets: ['latin'] })

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
      <body className={inter.className + ' body-container'}>
        <NavBar />
        {children}
      </body>
    </html>
  )
}
