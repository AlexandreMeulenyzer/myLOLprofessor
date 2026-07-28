export interface AccountRecord {
  puuid: string;
  gameName: string;
  tagLine: string;
  platform: string;
  isPrimary: boolean;
  linkedAt: string;
}

export const PLATFORMS: { value: string; label: string }[] = [
  { value: "euw1", label: "Europe de l'Ouest (EUW)" },
  { value: "eun1", label: "Europe Nordique & Est (EUNE)" },
  { value: "na1", label: "Amérique du Nord (NA)" },
  { value: "br1", label: "Brésil (BR)" },
  { value: "la1", label: "Amérique Latine Nord (LAN)" },
  { value: "la2", label: "Amérique Latine Sud (LAS)" },
  { value: "oc1", label: "Océanie (OCE)" },
  { value: "kr", label: "Corée (KR)" },
  { value: "jp1", label: "Japon (JP)" },
  { value: "tr1", label: "Turquie (TR)" },
  { value: "ru", label: "Russie (RU)" },
  { value: "ph2", label: "Philippines (PH)" },
  { value: "sg2", label: "Singapour (SG)" },
  { value: "th2", label: "Thaïlande (TH)" },
  { value: "tw2", label: "Taïwan (TW)" },
  { value: "vn2", label: "Vietnam (VN)" },
];
