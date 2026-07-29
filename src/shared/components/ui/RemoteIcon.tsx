import { useState } from "react";

/**
 * `<img>` charge depuis un CDN externe (Data Dragon/Community Dragon) qui se
 * masque silencieusement en cas d'echec plutot que d'afficher l'icone de lien
 * casse du navigateur (utile pour les assets recents pas toujours disponibles,
 * ex: emblemes de rang Emerald).
 */
export function RemoteIcon({
  src,
  alt,
  className,
}: {
  src: string;
  alt: string;
  className?: string;
}) {
  const [failed, setFailed] = useState(false);

  if (failed) {
    return null;
  }

  return (
    <img src={src} alt={alt} className={className} loading="lazy" onError={() => setFailed(true)} />
  );
}
