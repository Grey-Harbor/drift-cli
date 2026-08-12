import Link from 'next/link';

import { repositoryUrl } from '@/lib/seo';

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <div className="footer-links" aria-label="Related links">
        <Link href="/docs/tutorial">Build Drift CLI from source</Link>
        <a href="https://drift.greyharborsoftware.com" target="_blank" rel="noreferrer">
          Drift documentation
        </a>
        <a href={repositoryUrl} target="_blank" rel="noreferrer">
          GitHub repository
        </a>
        <a href="https://www.greyharborsoftware.com" target="_blank" rel="noreferrer">
          Grey Harbor Software
        </a>
      </div>
      <p>&copy; {new Date().getFullYear()} Grey Harbor Software. All rights reserved.</p>
    </footer>
  );
}
