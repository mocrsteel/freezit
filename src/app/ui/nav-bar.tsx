"use client"
import style from './navbar.module.scss'
import '@styles/globals.scss'
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import {faHouse, faSnowflake, faClipboardList, faCarrot, faGear} from '@fortawesome/free-solid-svg-icons'

import {useEffect, useState} from "react";
import {usePathname} from 'next/navigation'
import Link from "next/link";

const NavBar = () => {
  const currentPath = usePathname()
  return (
    <div className={style.navBar}>
      <Link
        href={'/'}
        id={'link-home'}
        className={
          style.navLink +
          (currentPath === '/' ? ' ' + style.navActive  : '')
        }
      >
        <div className={style.navIcon}>
          <FontAwesomeIcon icon={faHouse} size={"sm"}/>
        </div>
        <div className={style.navLabel}>
          Home
        </div>
      </Link>
      <Link
        href={'/storage'}
        id={'link-storage'}
        className={
          style.navLink +
          (currentPath === '/storage' ? ' ' + style.navActive : ' ')
        }
      >
        <div className={style.navIcon}>
          <FontAwesomeIcon icon={faClipboardList} size={"sm"} />
        </div>
        <div className={style.navLabel}>
          Storage
        </div>
      </Link>
      <Link
        href={'/products'}
        id={'link-products'}
        className={
          style.navLink +
          (currentPath === '/products' ? ' ' + style.navActive : ' ')
        }
      >
        <div className={style.navIcon}>
          <FontAwesomeIcon icon={faCarrot} size={"sm"} />
        </div>
        <div className={style.navLabel}>
          Products
        </div>
      </Link>
      <Link
        href={'/freezers'}
        id={'link-freezers'}
        className={
          style.navLink +
          (currentPath === '/freezers' ? ' ' + style.navActive : '')
        }
      >
        <div className={style.navIcon}>
          <FontAwesomeIcon icon={faSnowflake} size={"sm"}/>
        </div>
        <div className={style.navLabel}>
          Freezers
        </div>
      </Link>
      <Link
        href={'/settings'}
        id={'link-settings'}
        className={
          style.navLink +
          (currentPath === '/settings' ? ' ' + style.navActive : '')
        }
      >
        <div className={style.navIcon}>
          <FontAwesomeIcon icon={faGear} size={"sm"} />
        </div>
        <div className={style.navLabel}>
          Settings
        </div>
      </Link>
    </div>
  )
}

export default NavBar
