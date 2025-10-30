"use client"
import {FaHouse, FaSnowflake, FaClipboardList, FaCarrot, FaGear} from 'react-icons/fa6'
import type {IconType} from "react-icons";
import Link from "next/link";
import {usePathname} from 'next/navigation'
import React from "react";

interface NavLinkProps {
  title: string,
  href: string,
  icon: IconType,
  active?: boolean,
}

const navLinks: NavLinkProps[] = [
  {
    title: 'Home',
    href: '/',
    icon: FaHouse,
  },
  {
    title: 'Storage',
    href: '/storage',
    icon: FaClipboardList,
  },
  {
    title: 'Products',
    href: '/products',
    icon: FaCarrot,
  },
  {
    title: 'Freezers',
    href: '/freezers',
    icon: FaSnowflake,
  },
  {
    title: 'Settings',
    href: '/settings',
    icon: FaGear,
  },
]

function LinkItem({href, title, icon: Icon, active}: NavLinkProps) {
  return (
    <Link
      href={href}
      id={'link-' + title.toLowerCase()}
      className={
        "group transition-colors ease-out flex flex-col items-center justify-center w-1/5 sm:w-fit sm:h-fit py-2 sm:py-4 sm:px-6"
        + (
          active
            ? ' ' + 'transition-all ease-out bg-apple-500/90 text-white shadow-md shadow-apple-700/10'
            : ''
        )
      }
    >
      <div className="">
        <Icon/>
      </div>
      <div
        className='mt-0.5 text-xs sm:absolute sm:invisible sm:group-hover:delay-1000 sm:group-hover:visible sm:translate-y-10 sm:shadow-lg lg:translate-y-12 sm:p-1 sm:px-2 sm:rounded-xl sm:text-white sm:bg-slate-600/80 sm:before:absolute sm:before:bg-slate-600/80 sm:before:w-2 sm:before:h-2 sm:before:rotate-45 sm:before:left-0 sm:before:right-0 sm:before:mx-auto sm:before:z-50 sm:before:-top-1'>
        {title}
      </div>
    </Link>
  )
}

export default function NavBar() {
  const currentPath = usePathname()
  return (
    <div
      className="sticky w-full top-0 z-40 flex flex-row place-content-between sm:place-content-center shadow-sm border-b border-b-slate-700/10 backdrop-blur-md sm:backdrop-blur-lg bg-white/50">
      {navLinks.map(({title, href, icon}) => {
        const isCurrentPath = href === currentPath
        return <LinkItem key={title.toLowerCase()} href={href} title={title} icon={icon} active={isCurrentPath}/>
      })}
    </div>
  )
}
