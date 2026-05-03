

namespace ApiEndpoints
{
    public record ArenaNetApi
    {
        public string? BaseUri { get; init; }
        public string? Items { get; init; }
        public string? ItemStats { get; init; }
    };
}