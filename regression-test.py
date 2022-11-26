#!/usr/bin/env python3
from __future__ import annotations
import csv
from dataclasses import dataclass
import logging
import math
from pathlib import Path
import subprocess
import sys
import anyio
from anyio.streams.memory import MemoryObjectReceiveStream

log = logging.getLogger(__name__)

WORKERS = 8
TIMEOUT = 30


@dataclass
class TestCase:
    year: str
    problem: str
    input: str
    answer: str

    async def run(self) -> bool:
        log.info("RUNNING: %s-%s", self.year, self.problem)
        try:
            with anyio.fail_after(TIMEOUT):
                r = await anyio.run_process(
                    [
                        "cargo",
                        "run",
                        "-q",
                        "-r",
                        "-p",
                        f"advent-of-code-{self.year}-{self.problem}",
                        "--",
                        f"{self.year}/inputs/{self.input}",
                    ],
                    stdout=subprocess.PIPE,
                    stderr=None,
                )
        except TimeoutError:
            log.error("TIMEOUT: %s-%s", self.year, self.problem)
            return False
        except subprocess.CalledProcessError as e:
            log.error(
                "Problem %s-%s binary exited with return code %s",
                self.year,
                self.problem,
                e.returncode,
            )
            return False
        except Exception as e:
            log.error(
                "Problem %s-%s binary failed to execute: %s: %s",
                self.year,
                self.problem,
                type(e).__name__,
                str(e),
            )
            return False
        got = r.stdout.decode("utf-8").strip()
        if got == self.answer:
            log.info("PASS: %s-%s", self.year, self.problem)
            return True
        else:
            log.error("FAIL: %s-%s", self.year, self.problem)
            return False


def main() -> None:
    logging.basicConfig(
        format="[%(levelname)-8s] %(message)s",
        level=logging.DEBUG,
    )
    cases = []
    for p in Path().iterdir():
        if p.is_dir() and (p / "answers.csv").exists():
            year = p.name
            log.debug("Reading answers from %s", p / "answers.csv")
            with open(p / "answers.csv", newline="") as fp:
                reader = csv.DictReader(fp)
                cases.extend(
                    TestCase(
                        year=year,
                        problem=row["problem"],
                        input=row["input"],
                        answer=row["answer"],
                    )
                    for row in reader
                )
    ok = anyio.run(aruntests, cases)
    sys.exit(0 if ok else 1)


async def aruntests(cases: list[TestCase]) -> bool:
    ok = True

    async def dowork(rec: MemoryObjectReceiveStream[TestCase]) -> None:
        nonlocal ok
        async with rec:
            async for testcase in rec:
                if not await testcase.run():
                    ok = False

    async with anyio.create_task_group() as tg:
        sender, receiver = anyio.create_memory_object_stream(math.inf)
        async with receiver:
            for _ in range(WORKERS):
                tg.start_soon(dowork, receiver.clone())
        async with sender:
            for c in cases:
                await sender.send(c)
    return ok


if __name__ == "__main__":
    main()
