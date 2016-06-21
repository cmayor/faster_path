require "test_helper"
require "minitest/benchmark"
require "pathname"

class FasterPathBenchmark < Minitest::Benchmark
  def bench_rust_dirname
    assert_performance_constant do |n|
      5000.times do
        FasterPath.dirname "/quite/long/path/name/to/handle/readme.txt"
        FasterPath.dirname "/handle/readme.txt"
        FasterPath.dirname "/"
        FasterPath.dirname "."
        FasterPath.dirname "readme.txt"
        FasterPath.dirname ".txt"
        FasterPath.dirname "./txt"
      end
    end
  end


  def bench_ruby_dirname
    assert_performance_constant do |n|
      5000.times do
        File.dirname "/quite/long/path/name/to/handle/readme.txt"
        File.dirname "/handle/readme.txt"
        File.dirname "/"
        File.dirname "."
        File.dirname "readme.txt"
        File.dirname ".txt"
        File.dirname "./txt"
      end
    end
  end
end
