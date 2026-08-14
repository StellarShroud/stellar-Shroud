"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const links = [
  { href: "/", label: "Wallet" },
  { href: "/anchor", label: "Anchor" },
  { href: "/auditor", label: "Auditor" },
];

export function Nav() {
  const pathname = usePathname();

  return (
    <nav className="nav">
      <span className="nav-brand">StellarShroud</span>
      <div className="nav-links">
        {links.map((link) => (
          <Link
            key={link.href}
            href={link.href}
            className="nav-link"
            data-active={pathname === link.href}
          >
            {link.label}
          </Link>
        ))}
      </div>
    </nav>
  );
}
