import subprocess
import time

class Benchmark:
    def __init__(self, args, pairs):
        self.args = args
        self.pairs = pairs
        self.times = list()
        subprocess.run(['cargo', 'build'])

    def run(self):
        for problem, assumption in self.pairs:
            self.__run(problem, assumption)

    def __run(self, problem: str, assumption: str):

        # Warmup
        for i in range(self.args.warm_up_runs):
            subprocess.run(['cargo', 'run', '--', '-a', problem, assumption], capture_output=True)

        # Benchmark
        total_time = 0.0
        for i in range(self.args.num_runs):
            start = time.time()
            subprocess.run(['cargo', 'run', '--', '-a', problem, assumption], capture_output=True)
            end = time.time()
            elapsed_time = end - start
            total_time += elapsed_time
            print(f'Run {i+1}: {elapsed_time:.3f} seconds')

        avg_time = total_time / self.args.num_runs
        print(f'Average time: {avg_time:.3f} seconds')

        self.times.append(avg_time)

    def print_results(self):
        for i, time in enumerate(self.times):
            print(f'Test {i+1}: {time:.3f} seconds')
