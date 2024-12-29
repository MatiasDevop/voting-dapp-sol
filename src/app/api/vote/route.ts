import { ActionGetResponse, ActionPostRequest, ACTIONS_CORS_HEADERS } from "@solana/actions"
import { Connection, PublicKey } from "@solana/web3.js";
import { Voting } from "@/../anchor/target/types/voting";
import { BN, Program } from "@coral-xyz/anchor";

const IDL = require("@/../anchor/target/idl/voting.json");

export const OPTIONS = GET;

type LinkedActionType = "transaction" | "message" | "post" | "external-link";


export async function GET(request: Request) {
  const actionMetadata : ActionGetResponse = {
    icon: "https://acleanbake.com/wp-content/uploads/2014/05/How-to-Make-Peanut-Butter-Or-Another-Nut-or-Seed-Butter-10-720x720.jpg",
    title: "Vote for your favorite peanut butter",
    description: "Vote between Smooth and crunchy peanut butter",
    label: "Vote",
    links: {
      actions: [
        {
          type: "post",
          href: "/api/vote?candidate=Crunhcy",
          label: "Vote for crunchy",
        },
        {
          type: "post",
          href: "/api/vote?candidate=Smooth",
          label: "Vote for creamy",
        }
      ]
    }
  }
  return Response.json(actionMetadata, { headers: ACTIONS_CORS_HEADERS});
}

export async function POST(request: Request) {
  const url = new URL(request.url);
  const candidate = url.searchParams.get("candidate");
  if (candidate != "Crunchy" && candidate != "Smooth") {
    return new Response("Invalid candidate", { status: 400, headers: ACTIONS_CORS_HEADERS });
  } 

  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const program: Program<Voting> = new Program(IDL, { connection });

  const body: ActionPostRequest = await request.json();
  let voter;

  try {
    voter = new PublicKey(body.account);
  }catch(error) {
    return new Response("Invalid account", { status: 400, headers: ACTIONS_CORS_HEADERS });
  }

  const instruction = program.methods.vote(candidate, new BN(1));
}
