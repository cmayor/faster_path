require 'test_helper'

class EntriesTest < Minitest::Test
  def test_it_returns_similar_results_to_pathname_absolute?
    ['.', 'lib', 'src'].each do |pth|
      assert_equal Pathname.new(pth).entries.map(&:to_s).sort,
                   FasterPath.entries(pth).sort
    end
  end
end
